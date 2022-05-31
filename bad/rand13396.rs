
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13396(_: S3, _: S5, _: S7) {}
        
        fn test13396() { foo13396(S4, S0, S3, S2, S6); }
    