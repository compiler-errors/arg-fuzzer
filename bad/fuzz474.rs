
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo474(_: S2, _: S8) {}
        
        fn test474() { foo474(S1, S2, S5, S6, S7); }
    