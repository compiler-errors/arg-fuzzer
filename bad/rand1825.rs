
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1825(_: S6, _: S5, _: S6) {}
        
        fn test1825() { foo1825(S1, S2, S5, S8); }
    