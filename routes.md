| Feature        | Method | Path                                | Request body and others   | Authorization Header | Description                                         |
|----------------|--------|-------------------------------------|---------------------------|----------------------|-----------------------------------------------------|
| Authentication | POST   | /api/auth/login                     | username, password        | -                    | Login and get JWT token and Refresh token on Cookie |
|                | POST   | /api/auth/signup                    | username, email, password | -                    | Create new user                                     |
|                | POST   | /api/auth/logout                    | Cookie:refresh_token=abc  | -                    | Logout a user, revoke Refresh token and session     |
|                | POST   | /api/auth/refresh                   | Cookie:refresh_token=abc  | -                    | Rotate refresh token and Generaate new JWT token    |
| Users          | GET    | /api/users/me                       | &lt;empty&gt;             | Bearer &lt;token&gt; | Get current user profile                            |
|                | PUT    | /api/users/me                       | display_name, bio, ...    | Bearer &lt;token&gt; | Create or Update user profile                       |
|                | GET    | /api/users/me/following             | &lt;empty&gt;             | Bearer &lt;token&gt; | Get following users                                 |
|                | PUT    | /api/users/me/following/{target_id} | &lt;empty&gt;             | Bearer &lt;token&gt; | Follow other user                                   |
|                | DELETE | /api/users/me/following/{target_id} | &lt;empty&gt;             | Bearer &lt;token&gt; | Unfollow other user                                 |
|                | GET    | /api/users/{user_id}/following      | &lt;empty&gt;             | Bearer &lt;token&gt; | Get other user's following                          |
|                | GET    | /api/users/{user_id}/followers      | &lt;empty&gt;             | Bearer &lt;token&gt; | Get other user's followers                          |
|                |        |                                     |                           |                      |                                                     |
|                |        |                                     |                           |                      |                                                     |
|                |        |                                     |                           |                      |                                                     |
