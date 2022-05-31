
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13985(_: S7, _: S6) {}
        
        fn test13985() { foo13985(S1, S2, S3, S7, S8); }
    