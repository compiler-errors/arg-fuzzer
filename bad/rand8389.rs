
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8389(_: S1, _: S4) {}
        
        fn test8389() { foo8389(S6, S3, S3, S5, S6, S1, S7); }
    