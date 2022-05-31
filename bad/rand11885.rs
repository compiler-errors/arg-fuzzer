
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11885(_: S3, _: S4) {}
        
        fn test11885() { foo11885(S1, S3, S2, S1, S0); }
    