
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6376(_: S1, _: S4, _: S6) {}
        
        fn test6376() { foo6376(S2, S3, S4, S5, S8); }
    