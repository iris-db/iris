# Iris Query Language

Get all Users that are friends with a User with the username of Steve, and they are friends for more than 30 days.

```
# Powershell syntax
Get
	-Group "User"
	
	+IS_FRIENDS_WITH -> (
		-Group "User"
		-Where "username = 'Steve'"
		
		+IS_FRIENDS_WITH -> (
			-Group "User"
			-Where "username = 'Haberno'"
		)
	)
;
	
# SQL syntax
GET GROUP "User" MATCH 
	IS_FRIENDS_WITH -> (
		GROUP "User" WHERE "username = 'Steve'" LIMIT 3 MATCH
		
		IS_FRIENDS_WITH -> (
			GROUP "User" WHERE "username = 'Haberno'"
		)
	)
;
	
USE GRAPH "mygraph";
```
