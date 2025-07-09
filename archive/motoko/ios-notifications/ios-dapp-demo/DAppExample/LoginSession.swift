//
//  LoginSession.swift
//  DAppExample
//
//  Created by Kepler Vital on 20.12.22.
//

import SwiftUI
import Combine
import WebKit
import AuthenticationServices

public class LoginSession: NSObject, ObservableObject, ASWebAuthenticationPresentationContextProviding {
    private let parent: DAppWebView
    // callback used by the dapp to login the user
    private let authCallback: String
    private let callbackUrlParam = "callback_url" // https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app
    private let sessionKeyParam = "session" // the public key to be used
    private let restoreKey = "__ii"
    private let keypair = IdentityKeyPair.create()
    
    // Store for the open authentication session window, enabling interaction with the universal link
    open var session: ASWebAuthenticationSession?
    
    init(_ parent: DAppWebView) {
        self.parent = parent
        self.authCallback = parent.dapp.authCallbackURL
        
        super.init()
    }

    public func presentationAnchor(for session: ASWebAuthenticationSession) -> ASPresentationAnchor {
        return ASPresentationAnchor()
    }

    public func start() {
        var loginUrl = URL(string: parent.dapp.url.absoluteString)!
        loginUrl.append(queryItems: [URLQueryItem(name: callbackUrlParam, value: authCallback), URLQueryItem(name: sessionKeyParam, value: keypair.publicKey())])
        
        self.parent.dapp.loginSession = self
        session = ASWebAuthenticationSession(url: loginUrl, callbackURLScheme: nil, completionHandler: { (callbackURL, error) in
            guard error == nil else {
              print("Failed to authenticate")
              return
            }
        })

        session?.prefersEphemeralWebBrowserSession = false
        session?.presentationContextProvider = self
        session?.start()
    }
    
    public func identityCallbackHook(successURL: URL) {
        let delegationsValue = successURL.fragment()?.split(separator: self.restoreKey + "=")[0] ?? "";

        DispatchQueue.main.async {
            let restoreAuthScript = """
                window["\(self.restoreKey)"] = {
                    delegation: "\(delegationsValue)",
                    key: JSON.stringify(["\(self.keypair.publicKey())", "\(self.keypair.secret())"]),
                };
            """
            
            let userScript = WKUserScript(source: restoreAuthScript, injectionTime: .atDocumentStart, forMainFrameOnly: true)
            self.parent.webView.configuration.userContentController.addUserScript(userScript)

            self.parent.webView.window?.rootViewController?.present(self.parent.loading, animated: true)
            self.parent.webView.reload()
        }

        session?.cancel()
    }
}
