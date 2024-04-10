# API Documentation

## Important information
I'm not sure if I'll be able to renew the API server, but it's guaranteed to be available *until* **December 25, 2024**

## Domain
- olahbarbershop.codes

## Endpoints
- /services - Our services
  - https://olahbarbershop.codes/services
    ```json
    {
      "name": "The service name (String)",
      "description": "Description of the service (String)",
      "amount": "The price (Number)"
    }
    ```
- /contactinfo - All the ways to contact us
  - https://olahbarbershop.codes/contactinfo
    ```json
    {
      "left": "Contact method to be displayed on the left side (Contact)",
      "right": "(If name and link are empty, the line will only include the 'left' contact method) Contact to be displayed on the right side (Contact)"
    }
    ```
  - Contact type:
    ```json
    {
      "name": "The name of contact method(e.g. email, facebook etc.) (String)",
      "link": "Link for said contact method (String)"
    }
    ```
- /locations - Our locations
  - https://olahbarbershop.codes/locations
    ```json
    {
      "address": "The location's address (String)",
      "phone_number": "The location's telephone number (String)",
      "monday_to_thursday": "The location's business hours from Monday to Thursday in 24-hour time (String)",
      "friday": "The location's business hours on Friday in 24-hour time (String)",
      "saturday_to_sunday": "The location's business hours from Saturday to Sunday in 24-hour time (String)"
    }
    ```
- /notifications - App notifications about news and merch
  - https://olahbarbershop.codes/notifications
    ```json
    {
      "_id": "The notification ID (ObjectID)",
      "type": "The notification type, either 'news' or 'merch' (String)",
      "date": "The date of when the notification was created in YYYY-MM-DD format (String)",
      "description": "The notification body itself (String)"
    }
    ```
