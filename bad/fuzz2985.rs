
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2985(_: S2, _: S4, _: S6, _: S6, _: S5) {}
        
        fn test2985() { foo2985(S7, S3, S8, S2, S1, S7); }
    