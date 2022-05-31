
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo128(_: S2, _: S1, _: S7) {}
        
        fn test128() { foo128(S5, S5, S1, S5); }
    