
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8396(_: S1, _: S2) {}
        
        fn test8396() { foo8396(S6, S6, S6, S1, S3, S7); }
    