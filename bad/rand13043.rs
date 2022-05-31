
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13043(_: S3, _: S6, _: S7) {}
        
        fn test13043() { foo13043(S6, S5, S5, S1, S3, S2, S2); }
    