
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11748(_: S2, _: S4) {}
        
        fn test11748() { foo11748(S6, S7, S5, S5, S6, S2, S6); }
    