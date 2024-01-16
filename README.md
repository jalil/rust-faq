
RUST FAQ WEPAPP FOR FUN AND PROFIT

TO REGISTER
curl --location --request POST 'localhost:3030/registration' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "email": "new@email.com",
      "password": "cleartext"
}'

1. 


To get a token, we have to log in first:
$ curl --location --request POST 'localhost:3030/login' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "email": "new@email.com",
      "password": "cleartext"
}' "v2.local.Z9EaQ7lfPByBzKIySACj9HH8T8YLkx36aUSR2bUodwjoZzdpak6s-h8"?


Then we can create our very first question:
$ curl --location --request POST 'localhost:3030/questions' \
      --header 'Authorization:
v2.local.Z9EaQ7lfPByBzKIySACj9HH8T8YLkx36aUSR2bUodwjoZzdpak6s-h8' \
      --header 'Content-Type: application/json' \
      --data-raw '{
      "title": "How can I code better?",
      "content": "Any tips for a Junior developer?"
  }'
Question added?
