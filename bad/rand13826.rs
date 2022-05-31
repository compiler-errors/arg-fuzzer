
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13826(_: S1, _: S5, _: S7) {}
        
        fn test13826() { foo13826(S2, S3, S4, S5, S8); }
    