
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo562(_: S2, _: S1, _: S7, _: S7) {}
        
        fn test562() { foo562(S6, S3, S5, S1, S5, S6); }
    