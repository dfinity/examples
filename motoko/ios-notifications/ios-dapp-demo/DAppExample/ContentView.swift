//
//  ContentView.swift
//  WebViewNotification
//
//  Created by Kepler Vital on 08.12.22.
//

import SwiftUI
import WebKit

struct ContentView: View {
    @EnvironmentObject private var dapp: DappConfig
    
    var body: some View {
        DAppWebView(dapp: dapp)
            .onOpenURL{ url in
                guard url.scheme?.hasPrefix(dapp.scheme) ?? false else { return }
                
                let components = URLComponents(string: url.absoluteString)
                if (components?.host == "navigate") {
                    let navigateTo = components?.queryItems?.filter({$0.name == "to"}).first?.value
                    guard navigateTo != nil else { return }
                    
                    let deepLink = URL(string: navigateTo!)!
                    if dapp.isAllowedDeepLink(url: deepLink) {
                        dapp.url = deepLink
                    }
                }
            }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
