
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4416(_: S3, _: S4, _: S4, _: S6, _: S6, _: S5) {}
        
        fn test4416() { foo4416(S2, S2, S3, S4, S7, S1, S5, S1); }
    