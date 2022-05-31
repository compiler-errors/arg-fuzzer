
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2328(_: S4, _: S5, _: S7) {}
        
        fn test2328() { foo2328(S2, S3, S5, S8); }
    