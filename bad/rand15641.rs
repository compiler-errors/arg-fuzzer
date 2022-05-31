
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15641(_: S6, _: S2) {}
        
        fn test15641() { foo15641(S1, S6, S1, S1, S6, S0); }
    