
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4790(_: S2, _: S3, _: S4, _: S5, _: S6, _: S8) {}
        
        fn test4790() { foo4790(S1, S1, S8, S4, S6, S3, S4, S2); }
    