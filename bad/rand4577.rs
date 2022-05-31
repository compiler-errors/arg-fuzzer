
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4577(_: S8, _: S2, _: S1, _: S4) {}
        
        fn test4577() { foo4577(S7, S7, S2, S4, S6); }
    