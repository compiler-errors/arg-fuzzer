
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4788(_: S1, _: S3, _: S7) {}
        
        fn test4788() { foo4788(S1, S2, S3, S4, S5, S6); }
    