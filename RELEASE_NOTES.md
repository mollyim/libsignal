v0.104.0

- backups:
    - Support audio waveform and duration on FilePointer.
    - Support for hidden sticker packs and favorite stickers.
- Java: Renamed CreateLoginReceiptCredentialException to ReceiptCredentialException.
- Swift: Renamed the four SignalError `ReceiptCredentialError…` cases to `receiptCredentialError…`.
- New typed APIs:
  - UnauthProfilesService.getProfileKeyCredential
  - Messages.reportMessage
  - UnauthSubscriptionsService.getSubscriptionReceiptCredential (just `getReceiptCredential` for Kotlin)
- Internal: libsignal now uses Android Gradle Plugin 9.4.0 and Gradle 9.6.0.
