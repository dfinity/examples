//
//  KeyPairGenerator.swift
//  DAppExample
//
//  Created by Kepler Vital on 21.12.22.
//

import Foundation
import Sodium

public class IdentityKeyPair {
    private let _publicKey: String
    private let _secret: String
    
    private init() {
        let keypair = Sodium().sign.keyPair()!
        let publicKey = keypair.publicKey
        let privateKey = keypair.secretKey
        
        _publicKey = IdentityKeyPair.derEncode(key: publicKey).map { String(format: "%02x", $0) }.joined()
        _secret = privateKey.map { String(format: "%02x", $0) }.joined()
    }
    
    public static func derEncode(key: [UInt8]) -> [UInt8] {
        return [
            0x30, // SEQUENCE tag
            0x2a, // length (42 bytes)
            0x30, // SEQUENCE tag
            0x05, // length (5 bytes)
            0x06, // OBJECT IDENTIFIER tag
            0x03, // length (3 bytes)
            0x2b, 0x65, 0x70, // Ed25519 OID
            0x03, // BIT STRING tag
            0x21, // length
            0x00, // number of unused bits
        ] + key
    }
    
    public static func create() -> IdentityKeyPair {
        return IdentityKeyPair()
    }
    
    public func publicKey() -> String {
        return _publicKey
    }
    
    public func secret() -> String {
        return _secret
    }
}
