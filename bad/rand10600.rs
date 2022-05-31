
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10600(_: S7, _: S4, _: S0, _: S3) {}
        
        fn test10600() { foo10600(S1, S2, S3, S5, S6, S7, S8); }
    