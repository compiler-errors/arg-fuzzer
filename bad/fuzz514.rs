
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo514(_: S7, _: S5) {}
        
        fn test514() { foo514(S3, S6, S8, S7, S2, S2, S4); }
    