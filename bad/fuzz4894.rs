
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4894(_: S1, _: S2, _: S6) {}
        
        fn test4894() { foo4894(S6, S6, S6, S3, S6, S8, S1, S5); }
    