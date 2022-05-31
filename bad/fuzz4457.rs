
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4457(_: S1, _: S2, _: S3, _: S5, _: S6, _: S7) {}
        
        fn test4457() { foo4457(S1, S6, S1, S2, S1, S5, S1, S2); }
    