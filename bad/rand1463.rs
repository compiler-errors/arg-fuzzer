
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1463(_: S2, _: S8) {}
        
        fn test1463() { foo1463(S1, S2, S4, S5, S7); }
    