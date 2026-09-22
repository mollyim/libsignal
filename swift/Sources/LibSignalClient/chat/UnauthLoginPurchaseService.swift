//
// Copyright 2026 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

import Foundation

public protocol UnauthLoginPurchaseService: Sendable {
    /// Obtain a ZK receipt credential for a completed one-time login payment.
    /// The receipt credential can then be presented at registration.
    /// subsequent retries to create a login credential for the same ``purchaseIdentifier`` must use
    /// an identical ``receiptCredentialRequestContext``.
    ///
    /// - Throws:
    ///   - ``SignalError/ReceiptCredentialErrorPaymentRequired(_:)`` if the purchase is still pending with the payment provider. The client may retry later.
    ///   - ``SignalError/ReceiptCredentialErrorPaymentNotFound(_:)`` if the purchase did not complete successfully.
    ///   - ``SignalError/ReceiptCredentialErrorPaymentStillProcessing(_:)`` if the payment provider has no purchase with the provided ``purchaseIdentifier``
    ///   - ``SignalError/ReceiptCredentialErrorReceiptAlreadyIssued(_:)`` if the purchase was already redeemed for a receipt credential, but with a different receipt credential request
    ///   - the standard Signal network errors
    func createLoginReceiptCredential(
        paymentProcessor: PaymentProvider,
        purchaseIdentifier: String,
        receiptCredentialRequestContext: ReceiptCredentialRequestContext,
        serverParams: ServerPublicParams,
        purchaseTime: Date,
    ) async throws -> ReceiptCredential
}

extension UnauthenticatedChatConnection: UnauthLoginPurchaseService {
    public func createLoginReceiptCredential(
        paymentProcessor: PaymentProvider,
        purchaseIdentifier: String,
        receiptCredentialRequestContext: ReceiptCredentialRequestContext,
        serverParams: ServerPublicParams,
        purchaseTime: Date,
    ) async throws -> ReceiptCredential {
        return try await NativeNice.UnauthenticatedChatConnection_create_login_receipt_credential(
            asyncContext: self.tokioAsyncContext,
            chat: self,
            paymentProcessor: paymentProcessor,
            purchaseIdentifier: purchaseIdentifier,
            receiptCredentialRequestContext: receiptCredentialRequestContext,
            serverParams: serverParams,
            purchaseTime: purchaseTime,
        )
    }

}

extension UnauthServiceSelector where Self == UnauthServiceSelectorHelper<any UnauthLoginPurchaseService> {
    public static var loginPurchase: Self { .init() }
}
