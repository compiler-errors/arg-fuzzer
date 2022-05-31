
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15911(_: S3, _: S6, _: S7) {}
        
        fn test15911() { foo15911(S1, S3, S4, S5, S6, S8); }
    