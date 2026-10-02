v0.104.0

- backups: Support audio waveform and duration on FilePointer.
- Java: Renamed CreateLoginReceiptCredentialException to ReceiptCredentialException.
- Swift: Renamed the four SignalError `ReceiptCredentialError…` cases to `receiptCredentialError…`.
- New typed APIs:
  - UnauthProfilesService.getProfileKeyCredential
  - Messages.reportMessage
  - UnauthSubscriptionsService.getSubscriptionReceiptCredential (just `getReceiptCredential` for Kotlin)
