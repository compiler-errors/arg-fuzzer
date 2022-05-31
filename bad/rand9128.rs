
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9128(_: S2, _: S3, _: S7, _: S8) {}
        
        fn test9128() { foo9128(S0, S5, S7, S3, S2, S5, S7); }
    