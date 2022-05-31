
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6038(_: S3, _: S2, _: S6, _: S1, _: S4) {}
        
        fn test6038() { foo6038(S1, S8, S6, S5, S1, S7, S6, S3); }
    