
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2648(_: S1, _: S2) {}
        
        fn test2648() { foo2648(S7, S7, S5, S1, S4, S6, S3, S4); }
    