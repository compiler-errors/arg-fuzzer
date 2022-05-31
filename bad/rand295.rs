
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo295(_: S1, _: S7, _: S6, _: S2, _: S5, _: S8, _: S3, _: S4) {}
        
        fn test295() { foo295(S4, S1, S0, S0, S7); }
    