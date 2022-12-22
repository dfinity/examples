//
//  DappConfig.swift
//  DAppExample
//
//  Created by Kepler Vital on 14.12.22.
//

import SwiftUI
import Foundation

public class DappConfig: ObservableObject {
    static let shared = DappConfig()

    @Published open var loading = false
    @Published open var scheme = "dappexample"
    @Published open var url = URL(string: "https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app")!
    @Published open var authCallbackURL = "https://ptf55-faaaa-aaaag-qbd6q-cai.raw.ic0.app/mobile-auth-token"
    @Published open var deviceToken: String?

    open var loginSession: LoginSession?
    
    private let deeplinkAllowedURLs = [
        URL(string: "https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app")!.host
    ]
    
    public func isAllowedDeepLink(url: URL!) -> Bool {
        return deeplinkAllowedURLs.contains(url.host)
    }
}
