
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3128(_: S1, _: S5) {}
        
        fn test3128() { foo3128(S7, S3, S6, S1, S7, S3); }
    