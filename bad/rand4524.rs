
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4524(_: S1, _: S6, _: S7) {}
        
        fn test4524() { foo4524(S2, S3, S6, S1, S2, S3, S1); }
    