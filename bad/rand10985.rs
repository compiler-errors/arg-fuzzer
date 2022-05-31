
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10985(_: S1, _: S2) {}
        
        fn test10985() { foo10985(S7, S7, S3, S0, S1, S7, S6); }
    