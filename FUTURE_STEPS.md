# Future Steps for the Project

This document outlines planned features and improvements for the project.

## 1. Password Filtering and Organization

- **Minimum Length Filtering**

  - Implement logic to filter passwords based on a specified minimum length.
  - Example: if `minimumPasswordLength = 4`, only passwords with **4 or more characters** will be included.
  - Configurable via settings or input parameter.

- **Separation by Length**
  - Organize static passwords into separate lists based on their lengths.
  - Example:
    ```text
    4-char passwords: abcd, qwer, 1234
    5-char passwords: abcde, qwert, 12345
    ```
  - This improves readability and filtering efficiency.

---

## 2. IP Address Randomization

- **Fake IP Assignment**

  - Replace the real IP address with a **randomized fake IP** for every request.
  - Supports both IPv4 and IPv6 formats.
  - Ensures privacy and prevents tracking from repeated requests.

- **Implementation Considerations**
  - Use a secure random generator to produce realistic IPs.

---

## 3. Configuration Options

- Allow users to configure:
  - Minimum password length
  - Whether passwords should be grouped by length
  - IP randomization on/off
  - Custom static password lists
