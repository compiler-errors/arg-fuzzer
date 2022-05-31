
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2123(_: S2, _: S5, _: S6) {}
        
        fn test2123() { foo2123(S5, S5, S4, S2, S5, S4); }
    