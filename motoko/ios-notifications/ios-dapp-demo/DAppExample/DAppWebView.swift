//
//  WebView.swift
//  WebViewNotification
//
//  Created by Kepler Vital on 08.12.22.
//

import SwiftUI
import Combine
import WebKit
import AuthenticationServices

/// A container for using a WKWebView in SwiftUI
public struct DAppWebView: UIViewRepresentable {
    /// The dapp to display
    private let url: URL
    private let webView: WKWebView
    private let appScheme: String
    private let loading: LoadingScreen
  
    public init(dapp: DappConfig) {
        self.url = dapp.url
        self.appScheme = dapp.scheme

        let configuration = WKWebViewConfiguration()
        configuration.limitsNavigationsToAppBoundDomains = true
        configuration.preferences.javaScriptCanOpenWindowsAutomatically = true
        self.webView = WKWebView(frame: .zero, configuration: configuration)
        self.loading = LoadingScreen.create()
    }
  
    public func makeUIView(context: UIViewRepresentableContext<DAppWebView>) -> WKWebView {
        webView.uiDelegate = context.coordinator
        webView.navigationDelegate = context.coordinator

        return webView
    }
  
    public func updateUIView(_ uiView: WKWebView, context: UIViewRepresentableContext<DAppWebView>) {
        uiView.window?.rootViewController?.present(self.loading, animated: true)

        let request = URLRequest(url: self.url)
        uiView.load(request)
    }
    
    public func makeCoordinator() -> WebViewCoordinator {
        return WebViewCoordinator(self)
    }
    
    public class WebViewCoordinator: NSObject, WKUIDelegate, WKNavigationDelegate {
        let parent: DAppWebView
        
        init(_ parent: DAppWebView) {
            self.parent = parent

            super.init()
        }
        
        public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            parent.webView.window?.rootViewController?.dismiss(animated: false)
        }

        public func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
            parent.webView.window?.rootViewController?.dismiss(animated: false)
        }
        
        public func makeWKWebView(parentView: WKWebView, configuration: WKWebViewConfiguration, request: URLRequest) -> WKWebView {
            let view = WKWebView(frame: parentView.frame, configuration: configuration)
            parentView.addSubview(view)
            
            DispatchQueue.main.async {
                view.load(request)
            }
            return view
        }
        
        public class LoginSession: NSObject, ObservableObject, ASWebAuthenticationPresentationContextProviding {
            private let parent: DAppWebView
            // callback used by the dapp to login the user
            private let authCallback: String
            private let authQueryHandleParam = "ioshandle"
            private let identityProp = "_identity"
            
            init(_ parent: DAppWebView) {
                self.parent = parent
                self.authCallback = parent.appScheme + "://auth"

                super.init()
            }

            public func presentationAnchor(for session: ASWebAuthenticationSession) -> ASPresentationAnchor {
                return ASPresentationAnchor()
            }

            public func start() {
                var loginUrl = parent.url
                loginUrl.append(queryItems: [URLQueryItem(name: authQueryHandleParam, value: authCallback)])

                let authSession = ASWebAuthenticationSession(url: loginUrl, callbackURLScheme: parent.appScheme, completionHandler: { (callbackURL, error) in
                    guard error == nil, let successURL = callbackURL else {
                      print("Failed to authenticate")
                      return
                    }

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
                })
                
                authSession.prefersEphemeralWebBrowserSession = false
                authSession.presentationContextProvider = self
                authSession.start()
            }
        }
        
        public func webView(_ webView: WKWebView, createWebViewWith configuration: WKWebViewConfiguration, for navigationAction: WKNavigationAction, windowFeatures: WKWindowFeatures) -> WKWebView? {
            if (navigationAction.targetFrame == nil) {
                LoginSession(self.parent).start()
                
                return nil
            }
            
            return makeWKWebView(parentView: webView, configuration: configuration, request: navigationAction.request)
        }
    }
}
