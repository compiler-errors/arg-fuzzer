
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4801(_: S2, _: S7) {}
        
        fn test4801() { foo4801(S6, S2, S6, S1, S3, S2); }
    