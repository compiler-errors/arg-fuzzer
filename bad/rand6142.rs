
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6142(_: S2, _: S5, _: S6, _: S8) {}
        
        fn test6142() { foo6142(S2, S3, S2, S6, S2); }
    