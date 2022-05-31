
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8142(_: S4, _: S1, _: S0, _: S2) {}
        
        fn test8142() { foo8142(S3, S5, S4, S1, S6, S8, S7); }
    