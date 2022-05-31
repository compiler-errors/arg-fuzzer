
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13775(_: S1, _: S2) {}
        
        fn test13775() { foo13775(S3, S1, S5, S6, S3); }
    