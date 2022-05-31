
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13542(_: S1, _: S2, _: S3, _: S4, _: S5, _: S8) {}
        
        fn test13542() { foo13542(S7, S5, S3, S7, S1, S2, S3); }
    