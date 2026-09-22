//
// Copyright 2026 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

package org.signal.libsignal.net

public sealed class PaymentProvider {
  public data object GooglePlayBilling : PaymentProvider()

  public data object AppleAppStore : PaymentProvider()

  public data object Stripe : PaymentProvider()

  public data object Braintree : PaymentProvider()
}

public data class ChargeFailure(
  public val processor: PaymentProvider,
  public val code: String,
  public val message: String,
  public val outcomeNetworkStatus: String?,
  public val outcomeReason: String?,
  public val outcomeType: String?,
)
