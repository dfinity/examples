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
    private let authQueryHandleParam = "callback_url" // https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app
    private let identityProp = "_identity"
    
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
        var loginUrl = parent.dapp.url
        loginUrl.append(queryItems: [URLQueryItem(name: authQueryHandleParam, value: authCallback), URLQueryItem(name: "btn", value: "true")])
        
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
        let identityToken = NSURLComponents(string: (successURL.absoluteString))?.queryItems?.filter({$0.name == self.identityProp}).first?.value ?? ""

        DispatchQueue.main.async {
            let identityScript = """
                window['\(self.identityProp)'] = '\(identityToken)';
            """
            let userScript = WKUserScript(source: identityScript, injectionTime: .atDocumentStart, forMainFrameOnly: true)
            self.parent.webView.configuration.userContentController.addUserScript(userScript)

            self.parent.webView.window?.rootViewController?.present(self.parent.loading, animated: true)
            self.parent.webView.reload()
        }

        session?.cancel()
    }
}
