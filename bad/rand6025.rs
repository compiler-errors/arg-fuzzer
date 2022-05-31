
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6025(_: S2, _: S6, _: S3) {}
        
        fn test6025() { foo6025(S1, S2, S3, S4, S7, S8); }
    