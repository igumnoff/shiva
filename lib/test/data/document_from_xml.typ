#set page(
        header: "",
        footer: "",
    )
Page Header
header end
= 123
Hello, world!

            #image("Some titlepng", alt: "Some alt")
            
#link("http://example.com")
+ Item 1
+ Item 2
 - Item 2.1
- Item 2.2

        #table(
            columns:2,
            [*Name*],[*Salary*],
            [John Doe],[2000$],
[Marry Doe],[1000$],

        )
        Page Footer
footer end
