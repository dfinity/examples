"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.HttpAgent = exports.IdentityInvalidError = exports.RequestStatusResponseStatus = exports.makeNonce = void 0;
const principal_1 = require("@dfinity/principal");
const errors_1 = require("../../errors");
const auth_1 = require("../../auth");
const cbor = __importStar(require("../../cbor"));
const request_id_1 = require("../../request_id");
const buffer_1 = require("../../utils/buffer");
const transforms_1 = require("./transforms");
const types_1 = require("./types");
__exportStar(require("./transforms"), exports);
var types_2 = require("./types");
Object.defineProperty(exports, "makeNonce", { enumerable: true, get: function () { return types_2.makeNonce; } });
var RequestStatusResponseStatus;
(function (RequestStatusResponseStatus) {
    RequestStatusResponseStatus["Received"] = "received";
    RequestStatusResponseStatus["Processing"] = "processing";
    RequestStatusResponseStatus["Replied"] = "replied";
    RequestStatusResponseStatus["Rejected"] = "rejected";
    RequestStatusResponseStatus["Unknown"] = "unknown";
    RequestStatusResponseStatus["Done"] = "done";
})(RequestStatusResponseStatus = exports.RequestStatusResponseStatus || (exports.RequestStatusResponseStatus = {}));
// Default delta for ingress expiry is 5 minutes.
const DEFAULT_INGRESS_EXPIRY_DELTA_IN_MSECS = 5 * 60 * 1000;
// Root public key for the IC, encoded as hex
const IC_ROOT_KEY = '308182301d060d2b0601040182dc7c0503010201060c2b0601040182dc7c05030201036100814' +
    'c0e6ec71fab583b08bd81373c255c3c371b2e84863c98a4f1e08b74235d14fb5d9c0cd546d968' +
    '5f913a0c0b2cc5341583bf4b4392e467db96d65b9bb4cb717112f8472e0d5a4d14505ffd7484' +
    'b01291091c5f87b98883463f98091a0baaae';
// IC0 domain info
const IC0_DOMAIN = 'ic0.app';
const IC0_SUB_DOMAIN = '.ic0.app';
class HttpDefaultFetchError extends errors_1.AgentError {
    constructor(message) {
        super(message);
        this.message = message;
    }
}
class IdentityInvalidError extends errors_1.AgentError {
    constructor(message) {
        super(message);
        this.message = message;
    }
}
exports.IdentityInvalidError = IdentityInvalidError;
function getDefaultFetch() {
    let defaultFetch;
    if (typeof window !== 'undefined') {
        // Browser context
        if (window.fetch) {
            defaultFetch = window.fetch.bind(window);
        }
        else {
            throw new HttpDefaultFetchError('Fetch implementation was not available. You appear to be in a browser context, but window.fetch was not present.');
        }
    }
    else if (typeof global !== 'undefined') {
        // Node context
        if (global.fetch) {
            defaultFetch = global.fetch.bind(global);
        }
        else {
            throw new HttpDefaultFetchError('Fetch implementation was not available. You appear to be in a Node.js context, but global.fetch was not available.');
        }
    }
    else if (typeof self !== 'undefined') {
        if (self.fetch) {
            defaultFetch = self.fetch.bind(self);
        }
    }
    if (defaultFetch) {
        return defaultFetch;
    }
    throw new HttpDefaultFetchError('Fetch implementation was not available. Please provide fetch to the HttpAgent constructor, or ensure it is available in the window or global context.');
}
// A HTTP agent allows users to interact with a client of the internet computer
// using the available methods. It exposes an API that closely follows the
// public view of the internet computer, and is not intended to be exposed
// directly to the majority of users due to its low-level interface.
//
// There is a pipeline to apply transformations to the request before sending
// it to the client. This is to decouple signature, nonce generation and
// other computations so that this class can stay as simple as possible while
// allowing extensions.
class HttpAgent {
    constructor(options = {}) {
        this.rootKey = (0, buffer_1.fromHex)(IC_ROOT_KEY);
        this._pipeline = [];
        this._timeDiffMsecs = 0;
        this._rootKeyFetched = false;
        this._retryTimes = 3; // Retry requests 3 times before erroring by default
        this._isAgent = true;
 
        this._fetchOptions = options.fetchOptions; // added here, because options.source on line 127 - 135 doesn't run, if source is not given 
        this._callOptions = options.callOptions; // added here, because for CALL API, during fetch on line 233 you need {reactNative: {textStreaming: true}

        if (options.source) {   // logic here seems cyclical or faulty, even with a declared source as new HttpAgent() 
            console.log('here is source', options.source);

            if (!(options.source instanceof HttpAgent)) {
                throw new Error("An Agent's source can only be another HttpAgent");
            }
            this._pipeline = [...options.source._pipeline];
            this._identity = options.source._identity;
            this._fetch = options.source._fetch;
            this._fetchOptions = options.source._fetchOptions;
            this._host = options.source._host;
            this._credentials = options.source._credentials;
        }
        else {
            this._fetch = options.fetch || getDefaultFetch() || fetch.bind(global);
        }
        if (options.host !== undefined) {
            if (!options.host.match(/^[a-z]+:/) && typeof window !== 'undefined') {
                this._host = new URL(window.location.protocol + '//' + options.host);
            }
            else {
                this._host = new URL(options.host);
            }
        }
        else if (options.source !== undefined) {
            // Safe to ignore here.
            this._host = options.source._host;
        }
        else {
            const location = typeof window !== 'undefined' ? window.location : undefined;
            if (!location) {
                throw new Error('Must specify a host to connect to.');
            }
            this._host = new URL(location + '');
        }
        // Default is 3, only set if option is provided
        if (options.retryTimes !== undefined) {
            this._retryTimes = options.retryTimes;
        }
        // Rewrite to avoid redirects
        if (this._host.hostname.endsWith(IC0_SUB_DOMAIN)) {
            this._host.hostname = IC0_DOMAIN;
        }
        if (options.credentials) {
            const { name, password } = options.credentials;
            this._credentials = `${name}${password ? ':' + password : ''}`;
        }
        this._identity = Promise.resolve(options.identity || new auth_1.AnonymousIdentity());
        // Add a nonce transform to ensure calls are unique
        if (!options.disableNonce) {
            this.addTransform((0, transforms_1.makeNonceTransform)(types_1.makeNonce));
        }
        console.log('my options', options);
    }
    
    isLocal() {
        const hostname = this._host.hostname;
        return hostname === '127.0.0.1' || hostname.endsWith('localhost');
    }
    addTransform(fn, priority = fn.priority || 0) {
        // Keep the pipeline sorted at all time, by priority.
        const i = this._pipeline.findIndex(x => (x.priority || 0) < priority);
        this._pipeline.splice(i >= 0 ? i : this._pipeline.length, 0, Object.assign(fn, { priority }));
    }
    async getPrincipal() {
        if (!this._identity) {
            throw new IdentityInvalidError("This identity has expired due this application's security policy. Please refresh your authentication.");
        }
        return (await this._identity).getPrincipal();
    }
    async call(canisterId, options, identity) {
        const id = await (identity !== undefined ? await identity : await this._identity);
        if (!id) {
            throw new IdentityInvalidError("This identity has expired due this application's security policy. Please refresh your authentication.");
        }
        const canister = principal_1.Principal.from(canisterId);
        const ecid = options.effectiveCanisterId
            ? principal_1.Principal.from(options.effectiveCanisterId)
            : canister;
        const sender = id.getPrincipal() || principal_1.Principal.anonymous();
        let ingress_expiry = new transforms_1.Expiry(DEFAULT_INGRESS_EXPIRY_DELTA_IN_MSECS);
        // If the value is off by more than 30 seconds, reconcile system time with the network
        if (Math.abs(this._timeDiffMsecs) > 1000 * 30) {
            ingress_expiry = new transforms_1.Expiry(DEFAULT_INGRESS_EXPIRY_DELTA_IN_MSECS + this._timeDiffMsecs);
        }
        const submit = {
            request_type: types_1.SubmitRequestType.Call,
            canister_id: canister,
            method_name: options.methodName,
            arg: options.arg,
            sender,
            ingress_expiry,
        };
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        let transformedRequest = (await this._transform({
            request: {
                body: null,
                method: 'POST',
                headers: Object.assign({ 'Content-Type': 'application/cbor' }, (this._credentials ? { Authorization: 'Basic ' + btoa(this._credentials) } : {})),
            },
            endpoint: "call" /* Endpoint.Call */,
            body: submit,
        }));
        // Apply transform for identity.
        transformedRequest = await id.transformRequest(transformedRequest);
        const body = cbor.encode(transformedRequest.body);
        // Run both in parallel. The fetch is quite expensive, so we have plenty of time to
        // calculate the requestId locally.
        // ***  adds this._callOptions below which is essentially : {reactNative: {textStreaming: true}} to the call API
        const request = this._requestAndRetry(() => this._fetch('' + new URL(`/api/v2/canister/${ecid.toText()}/call`, this._host), Object.assign(Object.assign(this._callOptions, transformedRequest.request), { body })));
        const [response, requestId] = await Promise.all([request, (0, request_id_1.requestIdOf)(submit)]);
        return {
            requestId,
            response: {
                ok: response.ok,
                status: response.status,
                statusText: response.statusText,
            },
        };
    }
    async _requestAndRetry(request, tries = 0) {
        if (tries > this._retryTimes && this._retryTimes !== 0) {
            throw new Error(`AgentError: Exceeded configured limit of ${this._retryTimes} retry attempts. Please check your network connection or try again in a few moments`);
        }
        const response = await request();
        if (!response.ok) {
            const responseText = await response.clone().text();
            const errorMessage = `Server returned an error:\n` +
                `  Code: ${response.status} (${response.statusText})\n` +
                `  Body: ${responseText}\n`;
            if (this._retryTimes > tries) {
                console.warn(errorMessage + `  Retrying request.`);
                return await this._requestAndRetry(request, tries + 1);
            }
            else {
                throw new Error(errorMessage);
            }
        }
        return response;
    }
    async query(canisterId, fields, identity) {
        const id = await (identity !== undefined ? await identity : await this._identity);
        if (!id) {
            throw new IdentityInvalidError("This identity has expired due this application's security policy. Please refresh your authentication.");
        }
        const canister = typeof canisterId === 'string' ? principal_1.Principal.fromText(canisterId) : canisterId;
        const sender = (id === null || id === void 0 ? void 0 : id.getPrincipal()) || principal_1.Principal.anonymous();
        const request = {
            request_type: "query" /* ReadRequestType.Query */,
            canister_id: canister,
            method_name: fields.methodName,
            arg: fields.arg,
            sender,
            ingress_expiry: new transforms_1.Expiry(DEFAULT_INGRESS_EXPIRY_DELTA_IN_MSECS),
        };
        // TODO: remove this any. This can be a Signed or UnSigned request.
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        let transformedRequest = await this._transform({
            request: {
                method: 'POST',
                headers: Object.assign({ 'Content-Type': 'application/cbor' }, (this._credentials ? { Authorization: 'Basic ' + btoa(this._credentials) } : {})),
            },
            endpoint: "read" /* Endpoint.Query */,
            body: request,
        });
        // Apply transform for identity.
        transformedRequest = await (id === null || id === void 0 ? void 0 : id.transformRequest(transformedRequest));
        const body = cbor.encode(transformedRequest.body);
        const response = await this._requestAndRetry(() => this._fetch('' + new URL(`/api/v2/canister/${canister.toText()}/query`, this._host), Object.assign(Object.assign(Object.assign({}, this._fetchOptions), transformedRequest.request), { body })));
        return cbor.decode(await response.arrayBuffer());
    }
    async createReadStateRequest(fields, identity) {
        const id = await (identity !== undefined ? await identity : await this._identity);
        if (!id) {
            throw new IdentityInvalidError("This identity has expired due this application's security policy. Please refresh your authentication.");
        }
        const sender = (id === null || id === void 0 ? void 0 : id.getPrincipal()) || principal_1.Principal.anonymous();
        // TODO: remove this any. This can be a Signed or UnSigned request.
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const transformedRequest = await this._transform({
            request: {
                method: 'POST',
                headers: Object.assign({ 'Content-Type': 'application/cbor' }, (this._credentials ? { Authorization: 'Basic ' + btoa(this._credentials) } : {})),
            },
            endpoint: "read_state" /* Endpoint.ReadState */,
            body: {
                request_type: "read_state" /* ReadRequestType.ReadState */,
                paths: fields.paths,
                sender,
                ingress_expiry: new transforms_1.Expiry(DEFAULT_INGRESS_EXPIRY_DELTA_IN_MSECS),
            },
        });
        // Apply transform for identity.
        return id === null || id === void 0 ? void 0 : id.transformRequest(transformedRequest);
    }
    async readState(canisterId, fields, identity, 
    // eslint-disable-next-line
        request) {
        const canister = typeof canisterId === 'string' ? principal_1.Principal.fromText(canisterId) : canisterId;
        const transformedRequest = request !== null && request !== void 0 ? request : (await this.createReadStateRequest(fields, identity));
        const body = cbor.encode(transformedRequest.body); // insert next line: reactNative: { __nativeResponseType: "base64" }
        const response = await this._fetch('' + new URL(`/api/v2/canister/${canister}/read_state`, this._host), Object.assign(Object.assign(Object.assign({}, this._fetchOptions), transformedRequest.request), { body }));
        if (!response.ok) {
            throw new Error(`Server returned an error:\n` +
                `  Code: ${response.status} (${response.statusText})\n` +
                `  Body: ${await response.text()}\n`);
        }
        return cbor.decode(await response.arrayBuffer());
    }
    /**
     * Allows agent to sync its time with the network. Can be called during intialization or mid-lifecycle if the device's clock has drifted away from the network time. This is necessary to set the Expiry for a request
     * @param {PrincipalLike} canisterId - Pass a canister ID if you need to sync the time with a particular replica. Uses the management canister by default
     */
    async syncTime(canisterId) {
        const CanisterStatus = await Promise.resolve().then(() => __importStar(require('../../canisterStatus')));
        const callTime = Date.now();
        try {
            if (!canisterId) {
                console.log('Syncing time with the IC. No canisterId provided, so falling back to ryjl3-tyaaa-aaaaa-aaaba-cai');
            }
            const status = await CanisterStatus.request({
                // Fall back with canisterId of the ICP Ledger
                canisterId: canisterId !== null && canisterId !== void 0 ? canisterId : principal_1.Principal.from('ryjl3-tyaaa-aaaaa-aaaba-cai'),
                agent: this,
                paths: ['time'],
            });
            const replicaTime = status.get('time');
            if (replicaTime) {
                this._timeDiffMsecs = Number(replicaTime) - Number(callTime);
            }
        }
        catch (error) {
            console.error('Caught exception while attempting to sync time:', error);
        }
    }
    async status() {
        const headers = this._credentials
            ? {
                Authorization: 'Basic ' + btoa(this._credentials),
            }
            : {}; //insert next line: reactNative: { __nativeResponseType: "base64" }
        const response = await this._requestAndRetry(() => this._fetch('' + new URL(`/api/v2/status`, this._host), Object.assign({}, { headers }, this._fetchOptions)));
        return cbor.decode(await response.arrayBuffer());
    }
    async fetchRootKey() {
        if (!this._rootKeyFetched) {
            // Hex-encoded version of the replica root key
            this.rootKey = (await this.status()).root_key;
            this._rootKeyFetched = true;
        }
        return this.rootKey;
    }
    invalidateIdentity() {
        this._identity = null;
    }
    replaceIdentity(identity) {
        this._identity = Promise.resolve(identity);
    }
    _transform(request) {
        let p = Promise.resolve(request);
        for (const fn of this._pipeline) {
            p = p.then(r => fn(r).then(r2 => r2 || r));
        }
        return p;
    }
}
exports.HttpAgent = HttpAgent;
//# sourceMappingURL=index.js.map