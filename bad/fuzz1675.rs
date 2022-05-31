
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1675(_: S1, _: S2, _: S4, _: S6) {}
        
        fn test1675() { foo1675(S7, S8, S2, S6, S6, S2, S7, S6); }
    